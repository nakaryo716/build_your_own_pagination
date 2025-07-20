CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO articles (title, body, created_at)
VALUES
    ('Rust', 'Rust programming language is great', NOW() - interval '3 days'),
    ('TypeScript', 'TypeScript is useful', NOW() - interval '12 days'),
    ('Rust''s Ownership', 'Ownership is difficult. However, help us', NOW() - interval '7 days'),
    ('Haskell', 'Haskell is functional programming language', NOW() - interval '20 days'),
    ('Haskell', 'Monad is a feature', NOW() - interval '9 days'),
    ('TypeScript', 'React is super', NOW() - interval '1 day'),
    ('Scala', 'Scala is famous for actor system', NOW() - interval '15 days');
