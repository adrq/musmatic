/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright (c) 2019-2020 - Musmatic authors
 */

import React from 'react';
import SVGKeyboard from '../SVGKeyboard';
import './Toolbar.scss';

function Toolbar() {
  return (
    <div className="mainToolbar">
      <SVGKeyboard />
      <SVGKeyboard />
      <SVGKeyboard />
      <SVGKeyboard />
    </div>
  );
}

export default Toolbar;
