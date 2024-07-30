CREATE TABLE adresse (
  adresse_id UUID PRIMARY KEY,
  plz char(5) NOT NULL,
  strasse VARCHAR(255) NOT NULL,
  hausnr VARCHAR(255) NOT NULL,
  stadtteil VARCHAR(255) NOT NULL,
  CHECK (plz ~ '^[0-9]{5}$')
);

CREATE TABLE organisation (
  organisation_id UUID PRIMARY KEY,
  organisation_name VARCHAR(255) NOT NULL
);

CREATE TABLE ansprechpartner (
  ansprechpartner_id UUID PRIMARY KEY,
  nach_name VARCHAR(255) NOT NULL,
  vor_name VARCHAR(255) NOT NULL
);

CREATE TABLE email (
  email_id UUID PRIMARY KEY,
  email_address VARCHAR(255) UNIQUE NOT NULL,
  CHECK (
    email_address ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'
  )
);

CREATE TABLE telefonnummer (
  telefonnummer_id UUID PRIMARY KEY,
  land_vorwahl VARCHAR(5) NOT NULL,
  lokale_nummer VARCHAR(255) NOT NULL,
  festnetz BOOLEAN NOT NULL,
  komplette_nummer VARCHAR(255) GENERATED ALWAYS AS (land_vorwahl || ' ' || lokale_nummer) STORED
);

CREATE TABLE link (
  link_id UUID PRIMARY KEY,
  link VARCHAR(255) NOT NULL
);

CREATE TABLE angebot (
  angebot_id UUID PRIMARY KEY,
  angebot_name VARCHAR(255) NOT NULL,
  beschreibung VARCHAR(500),
  kategorie VARCHAR(255) NOT NULL,
  organisation_id UUID NOT NULL,
  created TIMESTAMPTZ NOT NULL,
  last_modified TIMESTAMPTZ NOT NULL,
  FOREIGN KEY (organisation_id) REFERENCES organisation(organisation_id)
);

CREATE TABLE sonstiges (
  sonstiges_id UUID PRIMARY KEY,
  text VARCHAR(255) NOT NULL,
  angebot_id UUID NOT NULL,
  FOREIGN KEY (angebot_id) REFERENCES angebot(angebot_id)
);

CREATE TABLE angebot_adresse (
  angebot_id UUID NOT NULL,
  adresse_id UUID NOT NULL,
  FOREIGN KEY (angebot_id) REFERENCES angebot(angebot_id),
  FOREIGN KEY (adresse_id) REFERENCES adresse(adresse_id)
);

CREATE TABLE angebot_apartner (
  angebot_id UUID NOT NULL,
  ansprechpartner_id UUID NOT NULL,
  FOREIGN KEY (angebot_id) REFERENCES angebot(angebot_id),
  FOREIGN KEY (ansprechpartner_id) REFERENCES ansprechpartner(ansprechpartner_id)
);

CREATE TABLE angebot_link (
  link_id UUID NOT NULL,
  angebot_id UUID NOT NULL,
  FOREIGN KEY (angebot_id) REFERENCES angebot(angebot_id),
  FOREIGN KEY (link_id) REFERENCES link(link_id)
);

CREATE TABLE apartner_email (
  ansprechpartner_id UUID NOT NULL,
  email_id UUID NOT NULL,
  FOREIGN KEY (ansprechpartner_id) REFERENCES ansprechpartner(ansprechpartner_id),
  FOREIGN KEY (email_id) REFERENCES email(email_id)
);

CREATE TABLE apartner_tnummer (
  ansprechpartner_id UUID NOT NULL,
  telefonnummer_id UUID NOT NULL,
  FOREIGN KEY (ansprechpartner_id) REFERENCES ansprechpartner(ansprechpartner_id),
  FOREIGN KEY (telefonnummer_id) REFERENCES telefonnummer(telefonnummer_id)
);

CREATE TABLE organisation_apartner (
  organisation_id UUID NOT NULL,
  ansprechpartner_id UUID NOT NULL,
  FOREIGN KEY (organisation_id) REFERENCES organisation(organisation_id),
  FOREIGN KEY (ansprechpartner_id) REFERENCES ansprechpartner(ansprechpartner_id)
);

CREATE TABLE organisation_adresse (
  organisation_id UUID NOT NULL,
  adresse_id UUID NOT NULL,
  FOREIGN KEY (organisation_id) REFERENCES organisation(organisation_id),
  FOREIGN KEY (adresse_id) REFERENCES adresse(adresse_id)
);

CREATE TABLE organisation_link (
  organisation_id UUID NOT NULL,
  link_id UUID NOT NULL,
  FOREIGN KEY (organisation_id) REFERENCES organisation(organisation_id),
  FOREIGN KEY (link_id) REFERENCES link(link_id)
);
