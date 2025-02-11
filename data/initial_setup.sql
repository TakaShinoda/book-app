INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'admin',
    'ts621@example.com',
    '$2b$12$RbDGVeKwWxE3EtDfSd9NVulGNdTKzeSSVKBuUBUW5AgRKUHTGbUKy',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
