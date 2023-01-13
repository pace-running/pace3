import { describe, expect, test } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import React from 'react';
import Footer from '.';

describe('checking footer', () => {
    beforeEach(() => {
        render(<Footer />);
    });

    test('checking links', () => {
        expect(screen.getByRole('link', { name: 'Datenschutz' })).toHaveAttribute('href', '/privacy_notice');
        expect(screen.getByRole('link', { name: 'Impressum' })).toHaveAttribute('href', '/imprint');
    });
});