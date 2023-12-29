import { ReactNode } from "react";

export interface ContainerProps {
  children?: ReactNode
}

export function Container(props: ContainerProps) {
  return (
    <div className="max-w-lg min-h-screen mx-auto">
      {props.children}
    </div>
  )
}
