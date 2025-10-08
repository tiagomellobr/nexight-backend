-- Create feeds table first (referenced by articles)
CREATE TABLE IF NOT EXISTS feeds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    feed_url VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    link VARCHAR(255) NOT NULL,
    last_build_date TIMESTAMPTZ,
    language VARCHAR(10),
    type VARCHAR(10) NOT NULL DEFAULT 'rss',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create articles table
CREATE TABLE articles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    link VARCHAR(255) NOT NULL,
    pub_date TIMESTAMPTZ NOT NULL,
    media VARCHAR(255),
    content TEXT NOT NULL,
    creator VARCHAR(255) NOT NULL,
    feed_id UUID NOT NULL REFERENCES feeds(id) ON DELETE CASCADE,
    ai_summary TEXT,
    rate INTEGER CHECK (rate >= 0 AND rate <= 10),
    keywords VARCHAR(255),
    processing_ai_summary BOOLEAN NOT NULL DEFAULT FALSE,
    processing_rating BOOLEAN NOT NULL DEFAULT FALSE,
    processing_keywords BOOLEAN NOT NULL DEFAULT FALSE,
    category_id UUID REFERENCES article_categories(id) ON DELETE SET NULL,
    processing_categorizing BOOLEAN NOT NULL DEFAULT FALSE,
    ai_columnist TEXT,
    processing_columnist BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX idx_articles_feed_id ON articles(feed_id);
CREATE INDEX idx_articles_category_id ON articles(category_id);
CREATE INDEX idx_articles_pub_date ON articles(pub_date DESC);
CREATE INDEX idx_articles_created_at ON articles(created_at DESC);
CREATE INDEX idx_articles_rate ON articles(rate);

-- Create index on link for duplicate detection
CREATE INDEX idx_articles_link ON articles(link);
