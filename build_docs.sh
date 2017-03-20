#!/bin/bash

set -e

cargo doc && mv target/doc docs
echo "<meta http-equiv=refresh content=0;url=futures_stop/index.html>" > docs/index.html
