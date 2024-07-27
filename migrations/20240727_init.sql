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

CREATE TABLE ansprech_partner (
  ansprech_partner_id UUID PRIMARY KEY,
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
  komplette_nummer VARCHAR(255) GENERATED ALWAYS AS (land_vorwahl || ' ' || lokale_nummer) STORED
);

CREATE TABLE angebot (
  angebot_id UUID PRIMARY KEY,
  angebot_name VARCHAR(255) NOT NULL,
  beschreibung VARCHAR(500),
  organisation_id UUID,
  created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (organisation_id) REFERENCES organisation(organisation_id)
);

CREATE TABLE angebot_adresse (
  angebot_id UUID,
  adresse_id UUID,
  FOREIGN KEY (angebot_id) REFERENCES angebot(angebot_id),
  FOREIGN KEY (adresse_id) REFERENCES adresse(adresse_id)
);

CREATE TABLE angebot_apartner(
  angebot_id UUID,
  ansprech_partner_id UUID,
  FOREIGN KEY (angebot_id) REFERENCES angebot(angebot_id),
  FOREIGN KEY (ansprech_partner_id) REFERENCES ansprech_partner(ansprech_partner_id)
);

CREATE TABLE apartner_email(
  ansprech_partner_id UUID,
  email_id UUID,
  FOREIGN KEY (ansprech_partner_id) REFERENCES ansprech_partner(ansprech_partner_id),
  FOREIGN KEY (email_id) REFERENCES email(email_id)
);

CREATE TABLE apartner_tnummer(
  ansprech_partner_id UUID,
  telefonnummer_id UUID,
  FOREIGN KEY (ansprech_partner_id) REFERENCES ansprech_partner(ansprech_partner_id),
  FOREIGN KEY (telefonnummer_id) REFERENCES telefonnummer(telefonnummer_id)
);
