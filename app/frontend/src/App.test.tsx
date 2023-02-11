/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright (c) 2019-2023 - Musmatic authors
 */
import React from 'react';
import { render, screen } from '@testing-library/react';
import App from './App';

test('renders containerToast', () => {
  render(<App />);
  const containerToast = screen.getByText(/App container/i);
  expect(containerToast).toBeInTheDocument();
});
