declare namespace JSX {
	export type Element = unknown;
}

declare module "@torque-rs/jsx-runtime" {
	declare function jsx(): JSX.Element;

	declare function Fragment(): JSX.Element;
}
