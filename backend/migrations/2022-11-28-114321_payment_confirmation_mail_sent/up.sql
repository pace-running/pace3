ALTER TABLE runners
ADD COLUMN payment_confirmation_mail_sent BOOLEAN NOT NULL DEFAULT false;