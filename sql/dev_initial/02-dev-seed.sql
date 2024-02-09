-- Root user (id = 0)
INSERT INTO "user" (
    id,
    username,
    email,
    first_name,
    last_name,
    typ,
    creator_id,
    creation_time,
    updater_id,
    updated_time
  )
VALUES (
    0,
    'root',
    'root@example.com',
    'Root',
    'User',
    'Sys',
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  );

-- Users demo1, demo2, demo3
INSERT INTO "user" (
    username,
    email,
    first_name,
    last_name,
    typ,
    creator_id,
    creation_time,
    updater_id,
    updated_time
  )
VALUES (
    'demo1',
    'demo1@example.com',
    'Demo',
    'One',
    'User',
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'demo2',
    'demo2@example.com',
    'John',
    'Doe',
    'User',
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'demo3',
    'demo3@example.com',
    'Jane',
    'Smith',
    'User',
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  );

-- Create authors associated with the newly inserted users
INSERT INTO author (
    user_id,
    typ,
    pen_name,
    bio,
    website,
    avatar_url,
    creation_time,
    updated_time,
    creator_id,
    updater_id
  )
VALUES (
    (
      SELECT id
      FROM "user"
      WHERE username = 'demo2'
    ),
    'Journalist',
    'JohnD',
    'Author bio goes here',
    'http://example.com',
    'http://example.com/avatar.jpg',
    NOW(),
    NOW(),
    0,
    0
  ),
  (
    (
      SELECT id
      FROM "user"
      WHERE username = 'demo3'
    ),
    'Editor',
    'JaneS',
    'Author bio goes here',
    'http://example.com',
    'http://example.com/avatar.jpg',
    NOW(),
    NOW(),
    0,
    0
  );