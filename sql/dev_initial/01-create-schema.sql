---- Base app schema
-- User
CREATE TYPE USER_TYPE AS ENUM ('Sys', 'Admin', 'User');

-- UserBmc
CREATE TABLE "user" (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  username VARCHAR(128) NOT NULL UNIQUE,
  email VARCHAR(255) NOT NULL UNIQUE,
  first_name VARCHAR(128) NOT NULL,
  last_name VARCHAR(128) NOT NULL,
  user_type USER_TYPE NOT NULL DEFAULT 'User',
  -- Auth
  pwd VARCHAR(256),
  pwd_salt UUID NOT NULL DEFAULT gen_random_uuid(),
  token_salt UUID NOT NULL DEFAULT gen_random_uuid(),
  -- Timestamps
  creator_id BIGINT NOT NULL,
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updater_id BIGINT NOT NULL,
  updated_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  -- Verification
  verificator_id BIGINT,
  verified_time TIMESTAMP WITH TIME ZONE
);

ALTER TABLE "user"
ADD CONSTRAINT fk_creator_id FOREIGN KEY (creator_id) REFERENCES "user"(id),
  ADD CONSTRAINT fk_updater_id FOREIGN KEY (updater_id) REFERENCES "user"(id);

-- Agent
-- AgentBmc
CREATE TABLE agent (
  -- PK
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  -- FKs
  owner_id BIGINT NOT NULL,
  -- Properties
  name VARCHAR(256) NOT NULL,
  ai_provider VARCHAR(256) NOT NULL DEFAULT 'dev',
  -- For now only support 'dev' provider
  ai_model VARCHAR(256) NOT NULL DEFAULT 'parrot',
  -- For now only support 'parrot' model
  -- Timestamps
  creator_id BIGINT NOT NULL,
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updater_id BIGINT NOT NULL,
  updated_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Conv
CREATE TYPE conv_kind AS ENUM ('OwnerOnly', 'MultiUsers');

CREATE TYPE conv_state AS ENUM ('Active', 'Archived');

-- ConvBmc
CREATE TABLE conv (
  -- PK
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  -- FKs
  owner_id BIGINT NOT NULL,
  agent_id BIGINT NOT NULL,
  -- Properties
  title VARCHAR(256),
  kind conv_kind NOT NULL DEFAULT 'OwnerOnly',
  state conv_state NOT NULL DEFAULT 'Active',
  -- Timestamps
  creator_id BIGINT NOT NULL,
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updater_id BIGINT NOT NULL,
  updated_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE conv
ADD CONSTRAINT fk_conv_agent FOREIGN KEY (agent_id) REFERENCES "agent"(id) ON DELETE CASCADE;

-- Conv Participants
-- ConvUserBmc
CREATE TABLE conv_user (
  -- PK
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  -- Properties / FKs
  conv_id BIGINT NOT NULL,
  user_id BIGINT NOT NULL,
  -- Machine User Properties
  auto_respond BOOLEAN NOT NULL DEFAULT FALSE,
  -- Timestamps
  creator_id BIGINT NOT NULL,
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updater_id BIGINT NOT NULL,
  updated_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Conv Messages
-- ConvMsgBmc
CREATE TABLE conv_msg (
  -- PK
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  -- FKs
  conv_id BIGINT NOT NULL,
  user_id BIGINT NOT NULL,
  -- Convert cid to user_id
  -- Properties
  content VARCHAR(1024) NOT NULL,
  -- Timestamps
  creator_id BIGINT NOT NULL,
  -- Convert cid to creator_id
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL,
  -- Convert ctime to creation_time
  updater_id BIGINT NOT NULL,
  updated_time TIMESTAMP WITH TIME ZONE NOT NULL -- Convert mtime to updated_time
);

ALTER TABLE conv_msg
ADD CONSTRAINT fk_conv_msg_conv FOREIGN KEY (conv_id) REFERENCES "conv"(id) ON DELETE CASCADE;

ALTER TABLE conv_user
ADD CONSTRAINT fk_conv_user_conv FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE;

-- Ganin news DB related 
-- CategoryBmc
CREATE TABLE category (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  name VARCHAR(256) NOT NULL,
  description TEXT,
  parent_id INTEGER REFERENCES category(id),
  -- Set default value to false
  is_featured BOOLEAN NOT NULL DEFAULT FALSE,
  -- Timestamp
  creator_id BIGINT NOT NULL,
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updater_id BIGINT NOT NULL,
  updated_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TYPE AUTHOR_TYPE AS ENUM (
  'Journalist',
  'Editor',
  'Columnist',
  'CopyWriter',
  'ContentWriter',
  'GhostWriter'
);

-- AuthorBmc
CREATE TABLE author (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  user_id BIGINT UNIQUE REFERENCES "user"(id),
  author_type AUTHOR_TYPE NOT NULL DEFAULT 'GhostWriter',
  pen_name VARCHAR(255) NOT NULL,
  bio TEXT,
  website VARCHAR(255),
  avatar_url VARCHAR(255),
  -- Timestamp
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  creator_id BIGINT REFERENCES "user"(id),
  updater_id BIGINT REFERENCES "user"(id)
);

CREATE TYPE APPROVAL_STATE AS ENUM (
  'Draft',
  'RequestApproval',
  'ApprovalPending',
  'Approved',
  'NeedCorrection',
  'Reject'
);

-- ArticleBmc
CREATE TABLE article (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  content TEXT NOT NULL,
  category_id INTEGER NOT NULL REFERENCES category(id),
  author_id BIGINT NOT NULL REFERENCES author(id),
  approval_state APPROVAL_STATE NOT NULL DEFAULT 'Draft',
  -- Approval
  approver_id SMALLINT,
  approval_time TIMESTAMP,
  -- Non Mandatory
  image_url VARCHAR(255),
  -- Timestamp
  creator_id BIGINT REFERENCES "user"(id),
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updater_id BIGINT REFERENCES "user"(id),
  updated_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CommentBmc
CREATE TABLE COMMENT (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  article_id BIGINT REFERENCES article(id),
  user_id BIGINT REFERENCES "user"(id),
  content TEXT NOT NULL,
  replay_to BIGINT REFERENCES COMMENT(id),
  -- Timestamp
  creator_id BIGINT REFERENCES "user"(id),
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updater_id BIGINT REFERENCES "user"(id),
  updated_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE category
ADD CONSTRAINT fk_category_parent_id FOREIGN KEY (parent_id) REFERENCES category(id);

ALTER TABLE article
ADD CONSTRAINT fk_article_category_id FOREIGN KEY (category_id) REFERENCES category(id);

ALTER TABLE COMMENT
ADD CONSTRAINT fk_comments_article_id FOREIGN KEY (article_id) REFERENCES article(id);

ALTER TABLE COMMENT
ADD CONSTRAINT fk_comments_user_id FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE;

-- SubscriptionBmc
CREATE TABLE subscription (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  subscriber BIGINT REFERENCES "user"(id),
  author_id BIGINT NOT NULL REFERENCES author(id),
  subscription_start_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  subscription_end_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP + INTERVAL '1 year',
  -- Timestamp
  creator_id BIGINT REFERENCES "user"(id),
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updater_id BIGINT REFERENCES "user"(id),
  updated_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- tag
CREATE TABLE tag (
  id INT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  name VARCHAR(256) NOT NULL,
  -- Timestamp
  creator_id BIGINT NOT NULL,
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tag_article (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  -- sementara pake tag_id, harusnya sih list tag. 
  tag_id INT REFERENCES tag(id),
  article_id BIGINT REFERENCES article(id),
  -- Timestamp
  creator_id BIGINT NOT NULL,
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE article_view (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  article_id BIGINT NOT NULL REFERENCES article(id),
  viewer_id BIGINT NOT NULL REFERENCES "user"(id),
  -- ini sepertinya bisa diupdate jika ngeview lagi dihari yang beda kali ya?
  view_count INT NOT NULL DEFAULT 1,
  likes BOOLEAN NOT NULL DEFAULT false,
  dislikes BOOLEAN NOT NULL DEFAULT false,
  SHARE BOOLEAN NOT NULL DEFAULT false,
  -- Timestamp
  creator_id BIGINT REFERENCES "user"(id),
  creation_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updater_id BIGINT REFERENCES "user"(id),
  updated_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);