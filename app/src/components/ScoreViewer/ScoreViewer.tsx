/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright (c) 2019-2020 - Musmatic authors
 */

import { IconButton } from '@chakra-ui/core';
import { AiOutlineZoomIn, AiOutlineZoomOut } from 'react-icons/ai';
import React, { useState, useEffect, useRef } from 'react';
import getElementWidth from '../../utils';
// import { Box } from '@chakra-ui/core';

function ScoreViewer() {
  const [svgWidth, setSvgWidth] = useState(100);
  const ref = useRef(null);
  function zoomIn() {
    const newSvgWidth = svgWidth + 100;
    console.log(`zoomIn ${svgWidth} -> ${newSvgWidth}`);
    setSvgWidth(newSvgWidth);
  }
  function zoomOut() {
    const newSvgWidth = svgWidth - 100;
    console.log(`zoomOut ${svgWidth} -> ${newSvgWidth}`);
    setSvgWidth(newSvgWidth);
  }

  // run once after first render to set initial svg width
  useEffect(() => {
    const initialWidth = getElementWidth(ref.current);
    console.log('width?', initialWidth);
    setSvgWidth(initialWidth - 100);
  }, []);

  useEffect(() => {
    function handleResize() {
      console.log('resized to: ', window.innerWidth, 'x', window.innerHeight);
    }

    window.addEventListener('resize', handleResize);
  });
  return (
    <div ref={ref}>
      <svg viewBox="0 0 84 40" xmlns="http://www.w3.org/2000/svg" style={{ width: svgWidth }}>
        <polygon data-note="0" className="ivory" points="0,0 7,0 7,20 12,20 12,40 0,40" style={{ fill: 'ivory', stroke: 'black' }} />
        <polygon data-note="1" points="7,0 14,0 14,20 7,20" style={{ fill: 'black', stroke: 'black' }} />
        <polygon data-note="2" className="ivory" points="14,0 21,0 21,20 24,20 24,40 12,40 12,20 14,20" style={{ fill: 'ivory', stroke: 'black' }} />
        <polygon data-note="3" points="21,0 28,0 28,20 21,20" style={{ fill: 'black', stroke: 'black' }} />
        <polygon data-note="4" className="ivory" points="28,0 36,0 36,40 24,40 24,20 28,20" style={{ fill: 'ivory', stroke: 'black' }} />
        <polygon data-note="5" className="ivory" points="36,0 42,0 42,20 48,20 48,40 36,40" style={{ fill: 'ivory', stroke: 'black' }} />
        <polygon data-note="6" points="42,0 49,0 49,20 42,20" style={{ fill: 'black', stroke: 'black' }} />
        <polygon data-note="7" className="ivory" points="49,0 56,0 56,20 60,20 60,40 48,40 48,20 49,20" style={{ fill: 'ivory', stroke: 'black' }} />
        <polygon data-note="8" points="56,0 63,0 63,20 56,20" style={{ fill: 'black', stroke: 'black' }} />
        <polygon data-note="9" className="ivory" points="63,0 70,0 70,20 72,20 72,40 60,40 60,20 63,20" style={{ fill: 'ivory', stroke: 'black' }} />
        <polygon data-note="10" points="70,0 77,0 77,20 70,20" style={{ fill: 'black', stroke: 'black' }} />
        <polygon data-note="11" className="ivory" points="77,0 84,0 84,40 72,40 72,20 77,20" style={{ fill: 'ivory', stroke: 'black' }} />
        <text x="2" y="37" fontFamily="sans-serif" fontWeight="bold" fontSize="6px" fill="blue">C</text>
      </svg>
      <IconButton aria-label="Zoom in" colorScheme="darkgray" size="lg" onClick={() => zoomIn()} icon={<AiOutlineZoomIn />} />
      <IconButton aria-label="Zoom out" colorScheme="darkgray" size="lg" onClick={() => zoomOut()} icon={<AiOutlineZoomOut />} />
    </div>
  );
}

export default ScoreViewer;
