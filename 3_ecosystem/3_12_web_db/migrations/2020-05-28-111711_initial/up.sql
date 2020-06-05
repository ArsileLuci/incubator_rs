-- Your SQL goes here
CREATE TABLE articles (
    id      VARCHAR     NOT NULL PRIMARY KEY,
    title   VARCHAR     NOT NULL,
    body    VARCHAR     NOT NULL
);

CREATE TABLE labels (
    id          INTEGER     NOT NULL PRIMARY KEY,
    name        VARCHAR     NOT NULL,
    article_id  VARCHAR     NOT NULL,
    FOREIGN KEY (article_id)     REFERENCES articles(id)
);
