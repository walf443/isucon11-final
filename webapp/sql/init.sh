#!/bin/bash
set -e -xu -o pipefail

CURRENT_DIR=$(cd $(dirname $0);pwd)
export MYSQL_HOST=${MYSQL_HOST:-127.0.0.1}
export MYSQL_PORT=${MYSQL_PORT:-3306}
export MYSQL_USER=${MYSQL_USER:-isucon}
export MYSQL_DATABASE=${MYSQL_DATABASE:-isucholar}
export MYSQL_PWD=${MYSQL_PWD:-isucon}
export LANG="C.UTF-8"
cd $CURRENT_DIR

# cat 0_setup.sql 1_schema.sql | mysql --defaults-file=/dev/null -h $MYSQL_HOST -P $MYSQL_PORT -u $MYSQL_USER $MYSQL_DATABASE
