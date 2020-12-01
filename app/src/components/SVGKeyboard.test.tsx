/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright (c) 2019-2020 - Musmatic authors
 */
import React from 'react';
import { render, screen, act } from '@testing-library/react';
import SVGKeyboard from './SVGKeyboard';

test('renders SVGKeyboard', () => {
    render(<SVGKeyboard />);
    const noteCinSVG = screen.getAllByText(/C/i);
    expect(noteCinSVG[0]).toBeInTheDocument();
});
