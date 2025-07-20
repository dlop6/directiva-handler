#!/bin/bash

# Configuración
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/backup"
LOG_DIR="/var/log/db-maintenance"
POSTGRES_DB="directiva_db"
POSTGRES_USER="directiva_user"
REDIS_CONTAINER="directiva-handler-redis-1"
PG_CONTAINER="directiva-handler-postgres-1"

# Crear directorios si no existen
mkdir -p "${BACKUP_DIR}/postgres"
mkdir -p "${BACKUP_DIR}/redis"
mkdir -p "$LOG_DIR"

# Función de logging
log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1" | tee -a "${LOG_DIR}/maintenance.log"
}

# Backup de PostgreSQL
log "Iniciando backup de PostgreSQL"
if pg_dump -h localhost -U "$POSTGRES_USER" "$POSTGRES_DB" > "${BACKUP_DIR}/postgres/backup_${TIMESTAMP}.sql"; then
    log "Backup de PostgreSQL completado exitosamente"
    # Mantener solo los últimos 7 backups
    find "${BACKUP_DIR}/postgres" -name "backup_*.sql" -type f -mtime +7 -delete
else
    log "ERROR: Fallo en el backup de PostgreSQL"
    exit 1
fi

# Vacuum y análisis de PostgreSQL
log "Iniciando VACUUM ANALYZE en PostgreSQL"
if psql -h localhost -U "$POSTGRES_USER" "$POSTGRES_DB" -c "VACUUM ANALYZE;"; then
    log "VACUUM ANALYZE completado exitosamente"
else
    log "ERROR: Fallo en VACUUM ANALYZE"
    exit 1
fi

# Backup de Redis
log "Iniciando backup de Redis"
if docker exec $REDIS_CONTAINER redis-cli SAVE && \
   docker cp $REDIS_CONTAINER:/data/dump.rdb "${BACKUP_DIR}/redis/dump_${TIMESTAMP}.rdb"; then
    log "Backup de Redis completado exitosamente"
    # Mantener solo los últimos 7 backups
    find "${BACKUP_DIR}/redis" -name "dump_*.rdb" -type f -mtime +7 -delete
else
    log "ERROR: Fallo en el backup de Redis"
    exit 1
fi

# Limpieza de Redis (ejecutar BGREWRITEAOF para compactar el AOF)
log "Iniciando compactación de Redis AOF"
if docker exec $REDIS_CONTAINER redis-cli BGREWRITEAOF; then
    log "Compactación de Redis AOF iniciada exitosamente"
else
    log "ERROR: Fallo al iniciar la compactación de Redis AOF"
    exit 1
fi

log "Mantenimiento de bases de datos completado exitosamente"
exit 0
