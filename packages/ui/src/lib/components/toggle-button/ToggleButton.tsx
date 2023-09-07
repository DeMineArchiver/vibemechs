import { For, ParentComponent, children, createEffect, createSignal, on } from "solid-js";

import styles from "./ToggleButton.module.css";
import { ReactiveSet } from "@solid-primitives/set";
import { ReactiveMap } from "@solid-primitives/map";

export interface ToggleButtonProps {
  /**
   * @default 0
   */
  initial?: number;
  onChange?: (selected: number, element: HTMLElement) => void;
}

const ToggleButton: ParentComponent<ToggleButtonProps> = (props) => {
  const resolved = children(() => props.children);
  const [selected, setSelected] = createSignal(props.initial ?? 0);

  const refs = new ReactiveMap<number, HTMLElement>();
  let indicator: HTMLElement | undefined;

  createEffect<number>((oldSelected) => {
    console.log(oldSelected);

    refs.get(oldSelected)?.classList.remove(styles["toggle-button-child-selected"]);

    const element = refs.get(selected());
    if(element && indicator) {
    element.classList.add(styles["toggle-button-child-selected"]);
    props.onChange?.(selected(), element);

      indicator.animate(
        {
          "left": `${element.offsetLeft}px`,
          "width": `${element.offsetWidth}px`
        },
        {
          duration: 250,
          fill: "forwards",
          easing: "cubic-bezier(.2,0,0,1)"
        }
      );
    }
    return selected();
  }, selected());
  
  return (
    <div class={styles["toggle-button"]}>
      <div
        ref={indicator as HTMLDivElement}
        class={styles["toggle-button-indicator"]} />
      <For each={resolved.toArray()}>
        {(child, index) => (
          <div
            ref={(element) => refs.set(index(), element)}
            class={styles["toggle-button-child"]}
            onClick={(event) => setSelected(index)}>
            {child}
          </div>
        )}  
      </For>
    </div>
  )
}

export default ToggleButton;
