/* eslint-disable jsx-a11y/heading-has-content */

import * as React from "react"
import { cn } from "~/lib/utils"

// https://ui.shadcn.com/docs/components/typography

export const H1 = React.forwardRef<
  HTMLHeadingElement,
  React.HTMLAttributes<HTMLHeadingElement>
>(({ className, ...props }, ref) => (
  <h1
    ref={ref}
    className={cn(
      "scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl",
      className
    )}
    {...props}
  />
))
H1.displayName = "H1"

export const H2 = React.forwardRef<
  HTMLHeadingElement,
  React.HTMLAttributes<HTMLHeadingElement>
>(({ className, ...props }, ref) => (
  <h1
    ref={ref}
    className={cn(
      "scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0",
      className
    )}
    {...props}
  />
))
H2.displayName = "H2"

export const H3 = React.forwardRef<
  HTMLHeadingElement,
  React.HTMLAttributes<HTMLHeadingElement>
>(({ className, ...props }, ref) => (
  <h1
    ref={ref}
    className={cn(
      "scroll-m-20 text-2xl font-semibold tracking-tight",
      className
    )}
    {...props}
  />
))
H3.displayName = "H3"
