-- Tabla de tipos de cuota
CREATE TABLE tipos_cuota (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    descripcion TEXT,
    es_extraordinaria BOOLEAN NOT NULL DEFAULT false
);

-- Tabla de cuotas
CREATE TABLE cuotas (
    id SERIAL PRIMARY KEY,
    usuario_id INTEGER NOT NULL REFERENCES usuarios(id_usuario),
    tipo_cuota_id INTEGER NOT NULL REFERENCES tipos_cuota(id),
    monto_cuota DECIMAL(15,2) NOT NULL,
    fecha_vencimiento DATE NOT NULL,
    monto_pagado DECIMAL(15,2) NOT NULL DEFAULT 0,
    multa DECIMAL(15,2) NOT NULL DEFAULT 0,
    fecha_creacion TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Tabla de moras
CREATE TABLE moras (
    id SERIAL PRIMARY KEY,
    usuario_id INTEGER NOT NULL REFERENCES usuarios(id_usuario),
    nombre_usuario VARCHAR(255) NOT NULL
);

-- Tabla de moras por cuota
CREATE TABLE moras_cuota (
    id SERIAL PRIMARY KEY,
    mora_id INTEGER NOT NULL REFERENCES moras(id),
    cuota_id INTEGER NOT NULL REFERENCES cuotas(id),
    mes_cuota VARCHAR(7) NOT NULL,
    monto DECIMAL(15,2) NOT NULL,
    estado estado_tipo NOT NULL DEFAULT 'Pendiente',
    fecha_creacion TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Tabla de moras por préstamo
CREATE TABLE moras_prestamo (
    id SERIAL PRIMARY KEY,
    mora_id INTEGER NOT NULL REFERENCES moras(id),
    prestamo_id INTEGER NOT NULL REFERENCES prestamos(id),
    prestamo_detalle_id INTEGER NOT NULL REFERENCES prestamos_detalle(id),
    mes_cuota VARCHAR(7) NOT NULL,
    monto DECIMAL(15,2) NOT NULL,
    estado estado_tipo NOT NULL DEFAULT 'Pendiente',
    fecha_creacion TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Índices
CREATE INDEX idx_cuotas_usuario ON cuotas(usuario_id);
CREATE INDEX idx_cuotas_tipo ON cuotas(tipo_cuota_id);
CREATE INDEX idx_cuotas_vencimiento ON cuotas(fecha_vencimiento);
CREATE INDEX idx_moras_usuario ON moras(usuario_id);
CREATE INDEX idx_moras_cuota_mora ON moras_cuota(mora_id);
CREATE INDEX idx_moras_cuota_cuota ON moras_cuota(cuota_id);
CREATE INDEX idx_moras_prestamo_mora ON moras_prestamo(mora_id);
CREATE INDEX idx_moras_prestamo_prestamo ON moras_prestamo(prestamo_id);

-- Datos iniciales para tipos de cuota
INSERT INTO tipos_cuota (nombre, descripcion, es_extraordinaria) VALUES
    ('Cuota Ordinaria', 'Cuota mensual regular', false),
    ('Cuota Extraordinaria', 'Cuota especial para proyectos específicos', true);
