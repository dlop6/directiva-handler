#!/bin/bash

# Configuración
MAINTENANCE_CONTAINER="directiva-handler-db-maintenance-1"
LOG_FILE="/var/log/db-maintenance/maintenance.log"
ALERT_WEBHOOK_URL="YOUR_WEBHOOK_URL"  # Reemplazar con tu webhook real

# Verificar el estado del contenedor
check_container() {
    if ! docker ps -q -f name=$MAINTENANCE_CONTAINER; then
        echo "ERROR: El contenedor de mantenimiento no está en ejecución"
        send_alert "El contenedor de mantenimiento no está en ejecución"
        exit 1
    fi
}

# Verificar los logs del último mantenimiento
check_logs() {
    # Obtener la última entrada del log
    LAST_LOG=$(docker exec $MAINTENANCE_CONTAINER tail -n 1 $LOG_FILE)
    
    if echo "$LAST_LOG" | grep -q "ERROR"; then
        echo "ERROR: Se encontró un error en el último mantenimiento"
        send_alert "Error en el mantenimiento: $LAST_LOG"
        exit 1
    fi
    
    # Verificar si el último mantenimiento fue hace más de 25 horas
    LAST_SUCCESS=$(docker exec $MAINTENANCE_CONTAINER grep "completado exitosamente" $LOG_FILE | tail -n 1)
    if [ -n "$LAST_SUCCESS" ]; then
        LAST_TIME=$(echo "$LAST_SUCCESS" | cut -d'[' -f2 | cut -d']' -f1)
        NOW=$(date +%s)
        LAST_TS=$(date -d "$LAST_TIME" +%s)
        DIFF=$((($NOW - $LAST_TS) / 3600))
        
        if [ $DIFF -gt 25 ]; then
            echo "ADVERTENCIA: El último mantenimiento exitoso fue hace más de 25 horas"
            send_alert "El último mantenimiento exitoso fue hace $DIFF horas"
            exit 1
        fi
    else
        echo "ERROR: No se encontraron registros de mantenimiento exitoso"
        send_alert "No hay registros de mantenimiento exitoso"
        exit 1
    fi
}

# Enviar alerta (reemplazar con tu sistema de alertas preferido)
send_alert() {
    if [ -n "$ALERT_WEBHOOK_URL" ]; then
        curl -X POST -H "Content-Type: application/json" \
             -d "{\"text\":\"$1\"}" \
             "$ALERT_WEBHOOK_URL"
    fi
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] ALERTA: $1" >> /var/log/db-maintenance/alerts.log
}

# Ejecutar verificaciones
check_container
check_logs

echo "Verificación completada: Todo está funcionando correctamente"
exit 0
