import { render, screen } from '@testing-library/react';
import MiniDrawer from './MiniDrawer';

test('renders learn react link', () => {
  render(<MiniDrawer />);
  const linkElement = screen.getByText(/learn react/i);
  expect(linkElement).toBeInTheDocument();
});
