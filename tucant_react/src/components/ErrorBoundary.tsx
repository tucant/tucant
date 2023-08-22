import { Component, ReactNode } from "react";

export class ErrorBoundary extends Component<{
  fallback: ReactNode;
  children: ReactNode;
}> {
  state = { hasError: false, error: null };
  static getDerivedStateFromError(error: unknown) {
    console.error(error);
    return {
      hasError: true,
      error,
    };
  }
  render() {
    if (this.state.hasError) {
      return (
        <div className="container">
          <div className="alert alert-success" role="alert">
            <h4 className="alert-heading">{this.props.fallback}</h4>
            <p>Die Fehlermeldung lautet: {String(this.state.error)}</p>
            <hr />
            <p className="mb-0">
              Eventuell hilft die Fehlermeldung einem Entwickler.
            </p>
          </div>
        </div>
      );
    }
    return this.props.children;
  }
}
