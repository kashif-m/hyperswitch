ALTER TABLE payment_intent
ADD COLUMN IF NOT EXISTS business_country VARCHAR(2) NOT NULL DEFAULT 'US',
    ADD COLUMN IF NOT EXISTS business_label VARCHAR(64) NOT NULL DEFAULT 'default';
