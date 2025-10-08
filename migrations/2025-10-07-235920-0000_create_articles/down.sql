-- Drop articles table and its indexes
DROP INDEX IF EXISTS idx_articles_link;
DROP INDEX IF EXISTS idx_articles_rate;
DROP INDEX IF EXISTS idx_articles_created_at;
DROP INDEX IF EXISTS idx_articles_pub_date;
DROP INDEX IF EXISTS idx_articles_category_id;
DROP INDEX IF EXISTS idx_articles_feed_id;
DROP TABLE IF EXISTS articles;

-- Drop feeds table
DROP TABLE IF EXISTS feeds;
