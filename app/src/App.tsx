/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright (c) 2019-2020 - Musmatic authors
 */

import React, { useRef } from 'react';
import { Flex } from '@chakra-ui/core';
import './App.scss';
import ScoreViewer from './components/ScoreViewer/ScoreViewer';

function App() {
  const parentEl = useRef(null);
  return (
    <Flex bg="darkgray" direction="column" color="white" ref={parentEl}>
      App container
      <ScoreViewer />
    </Flex>
  );
}

export default App;
