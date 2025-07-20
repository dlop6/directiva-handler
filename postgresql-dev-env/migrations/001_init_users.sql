-- Creación de tipos ENUM
CREATE TYPE rol_tipo AS ENUM ('directiva', 'socio', 'admin');
CREATE TYPE estado_tipo AS ENUM ('Vigente', 'Completado', 'Pendiente', 'Rechazado');

-- Tabla de usuarios
CREATE TABLE usuarios (
    id_usuario SERIAL PRIMARY KEY,
    nombre_completo VARCHAR(255) NOT NULL,
    correo VARCHAR(255) NOT NULL UNIQUE,
    dpi VARCHAR(13) NOT NULL UNIQUE,
    nit VARCHAR(10) NOT NULL,
    direccion TEXT NOT NULL,
    telefono VARCHAR(20) NOT NULL,
    total_aporte DECIMAL(15,2) NOT NULL DEFAULT 0,
    fecha_registro TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Tabla de roles
CREATE TABLE roles (
    id_rol SERIAL PRIMARY KEY,
    nombre rol_tipo NOT NULL,
    descripcion TEXT
);

-- Tabla de relación usuarios-roles
CREATE TABLE usuarios_roles (
    usuario_id INTEGER REFERENCES usuarios(id_usuario),
    rol_id INTEGER REFERENCES roles(id_rol),
    fecha_asignacion TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (usuario_id, rol_id)
);

-- Índices
CREATE INDEX idx_usuarios_correo ON usuarios(correo);
CREATE INDEX idx_usuarios_dpi ON usuarios(dpi);

-- Datos iniciales
INSERT INTO roles (nombre, descripcion) VALUES
    ('directiva', 'Miembro de la junta directiva'),
    ('socio', 'Socio regular de la cooperativa'),
    ('admin', 'Administrador del sistema');
