-- root user (at id = 0)
INSERT INTO "user" 
    (id,  typ, username, cid, ctime, mid, mtime) VALUES 
    (0, 'Sys', 'root',  0,   now(), 0,   now());

-- User demo1
INSERT INTO "user" 
    (username, cid, ctime, mid, mtime) VALUES 
    ('demo1',  0,   now(), 0,   now());
    ('demo2', 0, now(), 0, now()), 
    ('demo3', 0, now(), 0, now());
    
-- Create authors associated with the newly inserted users
INSERT INTO author (user_id, typ, full_name, pen_name, bio, website, avatar_url, created_at)
VALUES 
    ((SELECT id FROM "user" WHERE username = 'demo2'), 'Journalist', 'John Doe', 'JohnD', 'Author bio goes here', 'http://example.com', 'http://example.com/avatar.jpg', now()),
    ((SELECT id FROM "user" WHERE username = 'demo3'), 'Editor', 'Jane Smith', 'JaneS', 'Author bio goes here', 'http://example.com', 'http://example.com/avatar.jpg', now());

