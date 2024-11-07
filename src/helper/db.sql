create table risk (
    risk_uuid varchar(36) primary key,
    risk_name varchar(255) not null default 'New Risk',
    risk_description text not null
);

create table scenario (
    scenario_uuid varchar(36) primary key,
    risk_uuid varchar(36) not null,
    scenario_description text not null,
    threat_description text not null,
    add_note text
);

create table scenario_risk (
    scenario_uuid varchar(36) not null,
    likelihood int not null,
    reputation int not null,
    operational int not null,
    legal_compliance int not null,
    financial int not null
);

create table countermeasure(
    ctm_uuid varchar(36) primary key,
    scenario_uuid varchar(36) not null,
    title varchar(255) not null default 'New Countermeasure',
    description text not null,
    solved int not null default 0,
    solved_description text
);


-- Table for Missions
CREATE TABLE c1_mission (
    mission_id INT PRIMARY KEY AUTO_INCREMENT,
    mission_name VARCHAR(255) NOT NULL
);

-- Table for Business Values
CREATE TABLE c1_valeur_metier (
    valeur_id INT PRIMARY KEY AUTO_INCREMENT,
    mission_id INT,
    valeur_name VARCHAR(255) NOT NULL,
    valeur_nature VARCHAR(50),  -- e.g., 'Processus' or 'Information'
    valeur_description TEXT,
    responsable VARCHAR(255),   -- Responsible entity or person
);

-- Table for Associated Assets and Supports
CREATE TABLE c1_bien_support (
    support_id INT PRIMARY KEY AUTO_INCREMENT,
    valeur_id INT,
    support_name VARCHAR(255),
    support_description TEXT,
    support_responsable VARCHAR(255), -- Responsible entity or person for this support
);

CREATE TABLE c1_feared_event (
    event_id INT PRIMARY KEY AUTO_INCREMENT,
    valeur_metier int not null,
    evenement_redoute TEXT NOT NULL,
    impact TEXT NOT NULL,
    gravite INT NOT NULL
);

CREATE TABLE c1_gaps (
    gap_id INT PRIMARY KEY AUTO_INCREMENT,
    referential_type VARCHAR(255) NOT NULL,
    referential_name VARCHAR(255) NOT NULL,
    application_state int not null,
    gap TEXT,
    gap_justification TEXT,
    proposed_measures TEXT
);