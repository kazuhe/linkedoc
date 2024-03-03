"use client";
import React from "react";
import {
  Typography,
  List,
  ListItem,
  ListItemPrefix,
  ListItemSuffix,
  Chip,
  Accordion,
  AccordionHeader,
  AccordionBody,
} from "@material-tailwind/react";
import { Button } from "@/components/button";
import Link from "next/link";
import useSWR from "swr";

type Document = {
  id: number;
  path: string;
  title: string;
  description: string;
  tags: string[];
  created_at: string;
  updated_at: string;
};

const fetcher = async (url: string) => {
  const result = await fetch(url);
  return result.json();
};

export const Sidebar = () => {
  const [open, setOpen] = React.useState(0);

  // ドキュメント一覧
  const { data, error, isLoading } = useSWR<Document[]>(
    "http://localhost:8080/document",
    fetcher
  );

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>[getDocumentsError]Failed to load</div>;

  const handleOpen = (value: number) => {
    setOpen(open === value ? 0 : value);
  };

  return (
    <div className="w-full">
      <div className="p-4">
        <h1 className="text-xl font-black tracking-wider">Linkedoc</h1>
        <Link href="/documents/new">
          <Button
            size="sm"
            fullWidth={true}
            className="mt-2 flex items-center justify-center"
          >
            <img
              src="/svg/file-plus.svg"
              alt="New document"
              width={18}
              height={18}
            />
            <span className="ml-3">New document</span>
          </Button>
        </Link>
      </div>
      <List className="p-0">
        {data?.map((document: Document) => {
          return (
            <Link key={document.id} href={`/documents/${document.id}`}>
              <ListItem className="rounded-none px-4 text-sm text-white hover:bg-black-super-light hover:text-white">
                <ListItemPrefix>
                  <img
                    src="/svg/file.svg"
                    alt={document.title}
                    width={16}
                    height={16}
                  />
                </ListItemPrefix>
                {document.title}
                <ListItemSuffix>
                  <Chip
                    value="14"
                    size="sm"
                    className="text-white bg-black-light text-[10px]"
                  />
                </ListItemSuffix>
              </ListItem>
            </Link>
          );
        })}
        <hr className="my-2 border-black-light" />
        <Accordion
          open={open === 1}
          icon={<img src="/svg/chevron-down.svg" alt="Open" />}
        >
          <ListItem className="p-0" selected={open === 1}>
            <AccordionHeader
              onClick={() => handleOpen(1)}
              className="border-b-0 p-3"
            >
              <ListItemPrefix>
                {/* <PresentationChartBarIcon className="h-5 w-5" /> */}a
              </ListItemPrefix>
              <Typography color="blue-gray" className="mr-auto font-normal">
                Dashboard
              </Typography>
            </AccordionHeader>
          </ListItem>
          <AccordionBody className="py-1">
            <List className="p-0">
              <ListItem>
                <ListItemPrefix>
                  {/* <ChevronRightIcon strokeWidth={3} className="h-3 w-5" /> */}
                  a
                </ListItemPrefix>
                Analytics
              </ListItem>
              <ListItem>
                <ListItemPrefix>
                  {/* <ChevronRightIcon strokeWidth={3} className="h-3 w-5" /> */}
                  a
                </ListItemPrefix>
                Reporting
              </ListItem>
              <ListItem>
                <ListItemPrefix>
                  {/* <ChevronRightIcon strokeWidth={3} className="h-3 w-5" /> */}
                  a
                </ListItemPrefix>
                Projects
              </ListItem>
            </List>
          </AccordionBody>
        </Accordion>
        <Accordion
          open={open === 2}
          icon={<img src="/svg/chevron-down.svg" alt="Open" />}
        >
          <ListItem className="p-0" selected={open === 2}>
            <AccordionHeader
              onClick={() => handleOpen(2)}
              className="border-b-0 p-3"
            >
              <ListItemPrefix>
                {/* <ShoppingBagIcon className="h-5 w-5" /> */}a
              </ListItemPrefix>
              <Typography color="blue-gray" className="mr-auto font-normal">
                E-Commerce
              </Typography>
            </AccordionHeader>
          </ListItem>
          <AccordionBody className="py-1">
            <List className="p-0">
              <ListItem>
                <ListItemPrefix>
                  {/* <ChevronRightIcon strokeWidth={3} className="h-3 w-5" /> */}
                  s
                </ListItemPrefix>
                Orders
              </ListItem>
              <ListItem>
                <ListItemPrefix>
                  {/* <ChevronRightIcon strokeWidth={3} className="h-3 w-5" /> */}
                  a
                </ListItemPrefix>
                Products
              </ListItem>
            </List>
          </AccordionBody>
        </Accordion>
        <hr className="my-2 border-black-light" />
        <ListItem>
          <ListItemPrefix>
            {/* <InboxIcon className="h-5 w-5" /> */}a
          </ListItemPrefix>
          Inbox
        </ListItem>
        <ListItem>
          <ListItemPrefix>
            {/* <UserCircleIcon className="h-5 w-5" /> */}s
          </ListItemPrefix>
          Profile
        </ListItem>
        <ListItem>
          <ListItemPrefix>
            {/* <PowerIcon className="h-5 w-5" /> */}s
          </ListItemPrefix>
          Log Out
        </ListItem>
      </List>

      <div>return documentList: {JSON.stringify(data)}</div>
    </div>
  );
};
