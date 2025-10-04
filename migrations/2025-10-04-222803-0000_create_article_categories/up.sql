-- Create article_categories table
CREATE TABLE article_categories (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on name for faster lookups
CREATE INDEX idx_article_categories_name ON article_categories(name);

