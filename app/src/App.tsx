/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright (c) 2019-2020 - Musmatic authors
 */

import React, { useRef, useEffect } from 'react';
import { Flex } from '@chakra-ui/core';
import './App.scss';
import ScoreViewer from './components/ScoreViewer/ScoreViewer';

declare const window: any;

function App() {
  const parentEl = useRef(null);

  // HACK - wait for wasm to finish compiling/loading
  useEffect(() => {
    setTimeout(() => {
      console.log('Initializing verovio toolkit');
      const vrvToolkit = new window.verovio.toolkit();
      console.log(vrvToolkit);
    }, 2000);
  });
  return (
    <Flex bg="darkgray" direction="column" color="white" ref={parentEl}>
      App container
      <ScoreViewer />
    </Flex>
  );
}

export default App;
