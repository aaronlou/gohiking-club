-- Allow same user to post multiple reviews on the same event
ALTER TABLE event_reviews DROP CONSTRAINT IF EXISTS event_reviews_event_id_user_id_key;
