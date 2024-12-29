declare module "@torque-rs/ui" {
	declare class Window {
		private constructor();

		visible: boolean;
		title: string;

		static create(render: () => JSX.Element): Promise<Window>;
	}
}
