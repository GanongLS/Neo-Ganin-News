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
    author_type,
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
      WHERE username = 'demo1'
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
      WHERE username = 'demo2'
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
INSERT INTO category (
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
INSERT INTO article (
    title,
    content,
    category_id,
    author_id,
    approval_state,
    approver_id,
    approval_time,
    -- views,
    image_url,
    -- likes,
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
    'Draft',
    NULL,
    NULL,
    NULL,
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
    'Draft',
    NULL,
    NULL,
    NULL,
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
    'Draft',
    NULL,
    NULL,
    NULL,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,
    1002,
    1002 -- creator_id and updater_id
  ),
  (
    'Title of Article 4',
    'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam auctor tortor vitae mi sodales, vitae rhoncus elit suscipit. Nulla vel orci eu metus vestibulum malesuada et ac ex.',
    1000,
    1002,
    'Draft',
    NULL,
    NULL,
    NULL,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,
    1002,
    1002 -- creator_id and updater_id
  );

-- Comment for article with id 1001
INSERT INTO COMMENT (
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

-- Comment for article with id 1000
INSERT INTO COMMENT (
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

-- Comment for article with id 1002
INSERT INTO COMMENT (
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

INSERT INTO subscription (
    subscriber,
    author_id,
    subscription_start_time,
    subscription_end_time
  )
VALUES (
    1000,
    1001,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP + INTERVAL '1 year'
  ),
  (
    1002,
    1001,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP + INTERVAL '1 year'
  ),
  (
    1001,
    1001,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP + INTERVAL '1 year'
  );

-- Subscription 1,2,3
INSERT INTO subscription (
    subscriber,
    author_id,
    subscription_start_time,
    subscription_end_time,
    creation_time,
    updated_time,
    creator_id,
    updater_id
  )
VALUES (
    /* subscriber */
    1000,
    /* subscription_content */
    1001,
    /* subscription_start_time */
    CURRENT_TIMESTAMP,
    /* subscription_end_time */
    CURRENT_TIMESTAMP + INTERVAL '1 year',
    /* creation_time */
    CURRENT_TIMESTAMP,
    /* updated_time */
    CURRENT_TIMESTAMP,
    /* creator_id */
    0,
    /* updater_id */
    0
  ),
  (
    /* subscriber */
    1001,
    /* subscription_content */
    1001,
    /* subscription_start_time */
    CURRENT_TIMESTAMP,
    /* subscription_end_time */
    CURRENT_TIMESTAMP + INTERVAL '1 year',
    /* creation_time */
    CURRENT_TIMESTAMP,
    /* updated_time */
    CURRENT_TIMESTAMP,
    /* creator_id */
    0,
    /* updater_id */
    0
  ),
  (
    /* subscriber */
    1002,
    /* subscription_content */
    1001,
    /* subscription_start_time */
    CURRENT_TIMESTAMP,
    /* subscription_end_time */
    CURRENT_TIMESTAMP + INTERVAL '1 year',
    /* creation_time */
    CURRENT_TIMESTAMP,
    /* updated_time */
    CURRENT_TIMESTAMP,
    /* creator_id */
    0,
    /* updater_id */
    0
  );

INSERT INTO article_views (
    article_id,
    viewer_id,
    likes,
    dislikes,
    SHARE,
    creation_time,
    updated_time
  )
VALUES (
    1000,
    1002,
    FALSE,
    FALSE,
    FALSE,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  ),
  (
    1001,
    1001,
    FALSE,
    TRUE,
    FALSE,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  ),
  (
    1002,
    1000,
    TRUE,
    FALSE,
    FALSE,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  );