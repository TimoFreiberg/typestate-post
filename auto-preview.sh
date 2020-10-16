#!/bin/sh
set -e

echo typestate.adoc | entr asciidoctor /_
