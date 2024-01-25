/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright (c) 2019-2024 - Musmatic authors
 */
import React from 'react';
import { render, screen, act } from '@testing-library/react';
import Toolbar from './Toolbar';

test('renders Toolbar', () => {
    render(<Toolbar />);
    const noteCinSVG = screen.getAllByText(/C/i);
    expect(noteCinSVG[0]).toBeInTheDocument();
});
