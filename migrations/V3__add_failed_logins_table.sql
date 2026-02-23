CREATE TABLE failed_logins (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    ip_address VARCHAR(45) NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 1,
    locked_until TIMESTAMP WITH TIME ZONE,
    last_attempt_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE failed_logins
    ADD CONSTRAINT failed_logins_email_ip_unique UNIQUE (email, ip_address);

CREATE INDEX idx_failed_logins_email ON failed_logins(email);
CREATE INDEX idx_failed_logins_email_ip ON failed_logins(email, ip_address);
CREATE INDEX idx_failed_logins_locked_until ON failed_logins(locked_until);
