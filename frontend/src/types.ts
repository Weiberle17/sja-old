import { UUID } from "crypto"

export interface Angebot {
  angebot: {
    angebot_id: UUID;
    angebot_name: string;
    beschreibung: string;
    kategorie: string;
    organisation_id: UUID;
    created: Date;
    last_modified: Date;
  };
  organisation: Organisation;
  adressen: Adresse[];
  links: Link[];
  apartner: Ansprechpartner[];
  sonstiges: Sonstiges[];
}

export interface Organisation {
  organisation: {
    organisation_id: UUID;
    organisation_name: string;
  }
  adressen: Adresse[];
  links: Link[];
  apartner: Ansprechpartner[];
  angebot: {
    angebot_id: UUID;
    angebot_name: string;
    beschreibung: string;
    kategorie: string;
    organisation_id: UUID;
    created: Date;
    last_modified: Date;
  }
}

export interface Adresse {
  adresse_id: UUID;
  plz: string;
  strasse: string;
  hausnr: string;
  stadtteil: string;
}

export interface Link {
  link_id: UUID;
  link: string;
}

export interface Ansprechpartner {
  ansprechpartner_id: UUID;
  nach_name: string;
  vor_name: string;
}

export interface Sonstiges {
  sonstiges_id: UUID;
  text: string;
  angebot_id: UUID;
}
