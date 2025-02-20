/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright (c) 2019-2025 - Musmatic authors
 */

export function getElementWidth(el) {
  return el ? el.offsetWidth : 0;
}

export function getElementHeight(el) {
  return el ? el.offsetHeight : 0;
}
