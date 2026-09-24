-- AI chat history is kept locally alongside the rest of DevBreak data.
CREATE TABLE IF NOT EXISTS chat_conversations (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    title           TEXT NOT NULL,
    is_archived     INTEGER NOT NULL DEFAULT 0 CHECK (is_archived IN (0, 1)),
    created_at_utc  TEXT NOT NULL,
    updated_at_utc  TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_chat_conversations_visibility_updated
    ON chat_conversations(is_archived, updated_at_utc DESC);

CREATE TABLE IF NOT EXISTS chat_messages (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    conversation_id INTEGER NOT NULL,
    role            TEXT NOT NULL CHECK (role IN ('user', 'assistant')),
    content         TEXT NOT NULL,
    created_at_utc  TEXT NOT NULL,
    FOREIGN KEY(conversation_id) REFERENCES chat_conversations(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_chat_messages_conversation_created
    ON chat_messages(conversation_id, created_at_utc ASC);
