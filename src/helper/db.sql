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
    likehood int not null,
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