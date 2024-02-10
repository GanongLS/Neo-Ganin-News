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

-- Categories
INSERT INTO categories (
    name,
    description,
    parent_id,
    is_featured,
    creator_id,
    creation_time,
    updater_id,
    updated_time
  )
VALUES (
    'sport',
    NULL,
    NULL,
    FALSE,
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'politics',
    NULL,
    NULL,
    FALSE,
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'science',
    NULL,
    NULL,
    FALSE,
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'tech',
    NULL,
    NULL,
    FALSE,
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'art',
    NULL,
    NULL,
    FALSE,
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'business',
    NULL,
    NULL,
    FALSE,
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'law',
    NULL,
    NULL,
    FALSE,
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'criminal',
    NULL,
    NULL,
    FALSE,
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'health',
    NULL,
    NULL,
    FALSE,
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'education',
    NULL,
    NULL,
    FALSE,
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'culture',
    NULL,
    NULL,
    FALSE,
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  ),
  (
    'history',
    NULL,
    NULL,
    FALSE,
    0,
    CURRENT_TIMESTAMP,
    0,
    CURRENT_TIMESTAMP
  );

-- Articles
INSERT INTO articles (
    title,
    content,
    category_id,
    author_id,
    art_version,
    approval_state,
    approver_id,
    approval_time,
    publication_date,
    tags,
    is_featured,
    views,
    image_url,
    likes,
    creation_time,
    updated_time,
    creator_id,
    updater_id
  )
VALUES (
    'Title of Article 1',
    'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam auctor tortor vitae mi sodales, vitae rhoncus elit suscipit. Nulla vel orci eu metus vestibulum malesuada et ac ex.',
    1000,
    1000,
    -- Category_id and author_id
    1,
    'Draft',
    NULL,
    NULL,
    NULL,
    '{"tag1", "tag2", "tag3"}',
    false,
    0,
    NULL,
    0,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,
    1000,
    1000 -- creator_id and updater_id
  ),
  (
    'Title of Article 2',
    'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam auctor tortor vitae mi sodales, vitae rhoncus elit suscipit. Nulla vel orci eu metus vestibulum malesuada et ac ex.',
    1001,
    1001,
    -- Category_id and author_id
    1,
    'Draft',
    NULL,
    NULL,
    NULL,
    '{"tag1", "tag2", "tag3"}',
    false,
    0,
    NULL,
    0,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,
    1001,
    1001 -- creator_id and updater_id
  ),
  (
    'Title of Article 3',
    'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam auctor tortor vitae mi sodales, vitae rhoncus elit suscipit. Nulla vel orci eu metus vestibulum malesuada et ac ex.',
    1002,
    1002,
    -- Category_id and author_id
    1,
    'Draft',
    NULL,
    NULL,
    NULL,
    '{"tag1", "tag2", "tag3"}',
    false,
    0,
    NULL,
    0,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,
    1002,
    1002 -- creator_id and updater_id
  );

-- Comments for article with id 1001
INSERT INTO comments (
    article_id,
    user_id,
    content,
    creation_time,
    updated_time
  )
VALUES (
    1001,
    1000,
    'Lorem ipsum dolor sit amet, consectetur adipiscing elit.',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  ),
  (
    1001,
    1002,
    'Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  );

-- Comments for article with id 1000
INSERT INTO comments (
    article_id,
    user_id,
    content,
    creation_time,
    updated_time
  )
VALUES (
    1000,
    1002,
    'Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  ),
  (
    1000,
    1001,
    'Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  );

-- Comments for article with id 1002
INSERT INTO comments (
    article_id,
    user_id,
    content,
    creation_time,
    updated_time
  )
VALUES (
    1002,
    1001,
    'Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  ),
  (
    1002,
    1000,
    'Lorem ipsum dolor sit amet, consectetur adipiscing elit.',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  );