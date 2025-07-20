-- Tabla de préstamos
CREATE TABLE prestamos (
    id SERIAL PRIMARY KEY,
    solicitante_id INTEGER NOT NULL REFERENCES usuarios(id_usuario),
    nombre VARCHAR(255) NOT NULL,
    monto_total DECIMAL(15,2) NOT NULL,
    monto_cancelado DECIMAL(15,2) NOT NULL DEFAULT 0,
    motivo TEXT NOT NULL,
    tasa_interes DECIMAL(5,2) NOT NULL,
    fecha_solicitud DATE NOT NULL,
    plazo_meses INTEGER NOT NULL,
    meses_cancelados INTEGER NOT NULL DEFAULT 0,
    estado estado_tipo NOT NULL DEFAULT 'Pendiente',
    fecha_creacion TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Tabla de codeudores
CREATE TABLE codeudores (
    id SERIAL PRIMARY KEY,
    prestamo_id INTEGER NOT NULL REFERENCES prestamos(id),
    nombre VARCHAR(255) NOT NULL,
    correo VARCHAR(255) NOT NULL,
    dpi VARCHAR(13) NOT NULL,
    nit VARCHAR(10) NOT NULL,
    direccion TEXT NOT NULL,
    telefono VARCHAR(20) NOT NULL,
    fecha_registro TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Tabla de pagarés
CREATE TABLE pagares (
    id SERIAL PRIMARY KEY,
    prestamo_id INTEGER NOT NULL REFERENCES prestamos(id),
    pagare TEXT NOT NULL,
    estado estado_tipo NOT NULL DEFAULT 'Pendiente',
    comentarios_rechazo TEXT,
    fecha_creacion TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Tabla de detalles de préstamo (mensualidades)
CREATE TABLE prestamos_detalle (
    id SERIAL PRIMARY KEY,
    prestamo_id INTEGER NOT NULL REFERENCES prestamos(id),
    numero_cuota INTEGER NOT NULL,
    monto_cuota DECIMAL(15,2) NOT NULL,
    fecha_vencimiento DATE NOT NULL,
    monto_pagado DECIMAL(15,2) NOT NULL DEFAULT 0,
    multa DECIMAL(15,2) NOT NULL DEFAULT 0,
    UNIQUE(prestamo_id, numero_cuota)
);

-- Índices
CREATE INDEX idx_prestamos_solicitante ON prestamos(solicitante_id);
CREATE INDEX idx_codeudores_prestamo ON codeudores(prestamo_id);
CREATE INDEX idx_pagares_prestamo ON pagares(prestamo_id);
CREATE INDEX idx_prestamos_detalle_prestamo ON prestamos_detalle(prestamo_id);
CREATE INDEX idx_prestamos_estado ON prestamos(estado);
