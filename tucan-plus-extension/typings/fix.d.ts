export { }

declare global {
    interface RegExpConstructor {
        escape(str: string): string;
    }
}