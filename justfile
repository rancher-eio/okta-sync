#! /usr/bin/env -S just --justfile

PROG_CURL           := env_var_or_default("PROG_CURL", require("curl"))
PROG_FIND           := env_var_or_default("PROG_FIND", require("find"))
PROG_GRAPHQL_CLIENT := env_var_or_default("PROG_GRAPHQL_CLIENT", require("graphql-client"))
PROG_MKDIR          := env_var_or_default("PROG_MKDIR", require("mkdir"))
PROG_SORT           := env_var_or_default("PROG_SORT", require("sort"))
PROG_XARGS          := env_var_or_default("PROG_XARGS", require("xargs"))

GRAPHQL_FILE_EXTENSION   := env_var_or_default("GRAPHQL_FILE_EXTENSION", "graphql")
GRAPHQL_QUERY_DIRECTORY  := env_var_or_default("GRAPHQL_QUERY_DIRECTORY", "eio-okta-sync/src/github/graphql")
GRAPHQL_SCHEMA_DIRECTORY := env_var_or_default("GRAPHQL_SCHEMA_DIRECTORY", "eio-okta-sync/graphql")
GRAPHQL_SCHEMA_FILENAME  := env_var_or_default("GRAPHQL_SCHEMA_FILENAME", "schema.docs.graphql")
GRAPHQL_SCHEMA_ORIGIN    := env_var_or_default("GRAPHQL_SCHEMA_ORIGIN", "https://docs.github.com/public/ghec")

_default:
    {{ just_executable() }} --list

update-schema:
    {{ PROG_CURL }} \
      --create-dirs \
      --fail \
      --location \
      --output "{{ GRAPHQL_SCHEMA_DIRECTORY }}/{{ GRAPHQL_SCHEMA_FILENAME }}" \
      --silent \
      "{{ GRAPHQL_SCHEMA_ORIGIN }}/{{ GRAPHQL_SCHEMA_FILENAME }}"

@ensure-query-directory-exists:
    {{ PROG_MKDIR }} -p -v "{{ GRAPHQL_QUERY_DIRECTORY }}"

@find-queries:
    {{ PROG_FIND }} "{{ GRAPHQL_QUERY_DIRECTORY }}" -type f -name "*.{{ GRAPHQL_FILE_EXTENSION }}" \
      | {{ PROG_SORT }}

regenerate query: ensure-query-directory-exists
    {{ PROG_GRAPHQL_CLIENT }} generate \
      --fragments-other-variant \
      --response-derives="Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Serialize,Deserialize" \
      --schema-path="{{ GRAPHQL_SCHEMA_DIRECTORY }}/{{ GRAPHQL_SCHEMA_FILENAME }}" \
      --variables-derives="Debug,Clone,PartialEq,Eq,Hash,Deserialize,bon::Builder" \
      "{{ query }}"

regenerate-all:
    {{ just_executable() }} find-queries \
      | {{ PROG_XARGS }} --no-run-if-empty --max-args 1 {{ just_executable() }} regenerate