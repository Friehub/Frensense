// SAFE: collapsible is false (default) so at least one accordion panel must always remain open, ensuring mandatory content stays visible

import * as Accordion from '@radix-ui/react-accordion';
import { ChevronDownIcon } from '@radix-ui/react-icons';

export function TermsAccordion() {
  return (
    <Accordion.Root type="multiple">
      <Accordion.Item value="privacy">
        <Accordion.Header>
          <Accordion.Trigger>
            Privacy Policy <ChevronDownIcon />
          </Accordion.Trigger>
        </Accordion.Header>
        <Accordion.Content>Your data is handled per our privacy policy.</Accordion.Content>
      </Accordion.Item>
      <Accordion.Item value="terms">
        <Accordion.Header>
          <Accordion.Trigger>
            Terms of Service <ChevronDownIcon />
          </Accordion.Trigger>
        </Accordion.Header>
        <Accordion.Content>By using this service you agree to the terms.</Accordion.Content>
      </Accordion.Item>
      <Accordion.Item value="billing">
        <Accordion.Header>
          <Accordion.Trigger>
            Billing Summary <ChevronDownIcon />
          </Accordion.Trigger>
        </Accordion.Header>
        <Accordion.Content>You will be charged $49.99/month.</Accordion.Content>
      </Accordion.Item>
    </Accordion.Root>
  );
}
