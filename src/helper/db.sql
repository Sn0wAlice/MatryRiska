create table risk (
    risk_uuid varchar(36) primary key,
    risk_name varchar(255) not null default 'New Risk',
    risk_description text not null
);