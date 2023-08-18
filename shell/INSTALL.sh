#! /bin/bash

# Linux install script.
# --------------------
# Untested and unfinished.

PROJECT_NAME="hermod-server"
INSTALL_VER="0.4.0"

COLOR_RESET="\033[39;49m"
COLOR_RED="\033[31m"
COLOR_GREEN="\033[32m"
COLOR_GRAY="\033[90m"
COLOR_WHITE="\033[37m"

COLOR_BLUE="\033[94m"
COLOR_YELLOW="\e[93m"
# COLOR_RED="\e[31m"

STYLE_BOLD="\033[1m"
STYLE_RESET="\033[0m"

PROJECT_NAME="hermod-server"

# Ideal cases:
# - CentOS
# - Debian
# - Fedora Server
# Container: Fedora CoreOS

function info
{
    printf "$COLOR_BLUE$STYLE_BOLD     INFO$STYLE_RESET $1\n"
}

function warn
{
    printf "$COLOR_YELLOW$STYLE_BOLD     WARN$STYLE_RESET $1\n"
}

function error {
    printf "$COLOR_RED$STYLE_BOLD    ERROR$STYLE_RESET $1\n"
}

function ask
{
    printf "$COLOR_GREEN$STYLE_BOLD      ASK$STYLE_RESET $1"
    read -p "" i
}

# if [ "$USER" != "root" ]
# then
#     error "The bonfire install script needs to be run with root permissions."
#     info "If you do not trust this script for some reason, "
#     info "please follow our manual install guide or review it yourself."
#     exit 2
# fi

DATA_LOCATION="/home/hermodhost/"
# we also write to /srv/https/bonfire/

info "Data will be stored in $COLOR_WHITE$DATA_LOCATION"
# ew
#read -p "Is this okay? $COLOR_GRAY($COLOR_GREEN Y $COLOR_GRAY / $COLOR_RED n $COLOR_GRAY): $COLOR_WHITE" confirm && [[ $confirm == [yY] || $confirm == [yY][eE][sS] ]] || exit 1
ask "Is this okay? (Y/n): "

if [ $i != "y" ] && [ $i != "Y" ]
then
    info "Exiting ..."

    exit 1
fi

# if path_okay=false: 
#     read -p " Then what path would you like?: " DATA_LOCATION
# fi

info "Creating Server Hosting User"

# -g automated
useradd --create-home hermodhost

info "Cleaning defaults"

rm -rf /home/hermodhost/.mozilla
rm -rf /home/hermodhost/.bash_logout
rm -rf /home/hermodhost/.bash_profile

touch /home/hermodhost/.bashrc
echo 'export PS1="\[\e[36m\]\A\[\e[m\] \h \[\e[37;41m\]LIVE SERVER - PRODUCTION\[\e[m\]\[\e[31m\] \[\e[m\] "' >| /home/hermodhost/.bashrc

info "Creating Directories"

mkdir /home/hermodhost/data/
mkdir /home/hermodhost/data/dim/   # DIM data
mkdir /home/hermodhost/data/http/  # HTTPS/non-DIM data
mkdir /home/hermodhost/old/        # Old revisions to easily revert

info "Setting Permissions"

chmod 600 /home/hermodhost/data/ # Only hermodhost reads/writes, nothing for anyone else
chmod 044 /home/hermodhost/old/  # No touching for hermodhost, others can read

# mkdir /srv/$PROJECT_NAME/
# mkdir /srv/$PROJECT_NAME/app/
# mkdir /srv/$PROJECT_NAME/data/
# mkdir /srv/$PROJECT_NAME/https/
# mkdir /srv/$PROJECT_NAME/old/