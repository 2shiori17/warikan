import { faker } from '@faker-js/faker';

export type User = {
  id: string;
  name: string;
};

export type Group = {
  id: string;
  created_at: number;
  name: string;
  users: User[];
  payments: Payment[];
};

export type Payment = {
  id: string;
  created_at: number;
  name: string;
  creditor: User;
  debtors: User[];
}

export const fakeUser = (): User => ({
  id: faker.string.uuid(),
  name: faker.internet.userName(),
});

export const fakeGroup = (): Group => ({
  id: faker.string.uuid(),
  created_at: faker.date.anytime().getTime(),
  name: faker.lorem.word(),
  users: [fakeUser(), fakeUser(), fakeUser()],
  payments: [fakePayment(), fakePayment(), fakePayment()],
})

export const getGroup = async (id: string): Promise<Group> => ({
  id,
  created_at: faker.date.anytime().getTime(),
  name: faker.lorem.word(),
  users: [fakeUser(), fakeUser(), fakeUser()],
  payments: [fakePayment(), fakePayment(), fakePayment()],
})

export const fakePayment = (): Payment => ({
  id: faker.string.uuid(),
  created_at: faker.date.anytime().getTime(),
  name: faker.lorem.word(),
  creditor: fakeUser(),
  debtors: [fakeUser(), fakeUser(), fakeUser()],
})

export const getPayment = async (id: string): Promise<Payment> => ({
  id,
  created_at: faker.date.anytime().getTime(),
  name: faker.lorem.word(),
  creditor: fakeUser(),
  debtors: [fakeUser(), fakeUser(), fakeUser()],
})
