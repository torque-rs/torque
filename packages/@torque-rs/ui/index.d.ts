declare module "@torque-rs/ui" {
	declare class Node {}

	declare class Element extends Node {}

	declare class Window {
		private constructor();

		visible: boolean;
		title: string;

		static create(render: () => JSX.Element): Promise<Window>;

		createElement(): Element;
	}
}
