import { EditIcon } from "@chakra-ui/icons";
import { Box, BoxProps, HStack, Heading, Skeleton, Table, Tbody, Td, Th, Thead, Tr, Text, IconButton } from "@chakra-ui/react";
import React, { useCallback, useState } from "react";
import { UpdatePinStatus } from "./UpdatePinStatus";

export interface TableProps extends Omit<BoxProps, "onClick"> {
  /** Table data */
  data: Record<string, any>[] | null;
  /** Table headers and mapping to record keys */
  headers: { key: string; label: string; skeletonWidth?: number }[];
  /** Callback when row is clicked */
  onClick?: (item: Record<string, any>, index: number) => void;
  /** Label to be used when displaying "no data available" message */
  label?: string;
  /** Styling variant to use for the rows */
  rowVariant?: string;
  // number of rows to be added while the table is loading
  loadingRows?: number
}

const TableView = ({
  data,
  headers,
  onClick,
  label = "data",
  rowVariant = "diamondStriped",
  loadingRows = 0,
  ...props
}: TableProps) => {
  const handleClick = useCallback(
    (row: React.MouseEvent<HTMLTableRowElement>) => {
      if (onClick && data) {
        const target = (row.target as HTMLTableCellElement).dataset.id;
        if (target) {
          const intTarget = parseInt(target);
          onClick(data[intTarget], intTarget);
        }
      }
    },
    [data, onClick],
  );

  const [show, setShow] = useState(true);

  return (
    <Box overflowY='scroll' {...props}>
      {data === null || data.length === 0 ? (
        <Heading py={10} w='100%' variant='notFound'>
          No {label.toLowerCase()} found
        </Heading>
      ) : (
        <Table size='sm' variant={rowVariant}>
          <Thead>
            <Tr>
              {headers.map((header) => (
                <Th key={header.label}>{header.label}</Th>
              ))}
            </Tr>
          </Thead>
          
          <Tbody cursor='pointer'>
            {data.map((item, i) => (
              <Tr h='2vh' key={i} onClick={handleClick}>
                {headers.map((header) => (
                  <Td data-id={i} key={header.key}>
                    <HStack spacing={2}>
                      <Text>
                    {item[header.key]}
                    </Text>
                    <Text>
                      {header.key === 'status' ? 
                          <IconButton 
                            aria-label='editButton' 
                            icon={<EditIcon color={'white'}/>} 
                            colorScheme='black' 
                            variant={'unstyled'}
                            onClick={() => setShow(!show)}
                          />  
                        : null}
          
                    </Text>
                    </HStack>
                    <Text>
                    {header.key === 'status' && show ? <UpdatePinStatus item={item}/> : null}
                    </Text>
                  </Td>
                ))}
              </Tr>
            ))}
            
            {[...Array(loadingRows)].map(() => (
              <Tr h='2vh'>
                {headers.map((header) => (
                  <Td key={header.key}>
                    <Skeleton height='1em' width={`${header.skeletonWidth}ch`}/>
                  </Td>
                ))}
              </Tr>
            ))}
          </Tbody>
          
        </Table>
      )}
    </Box>
  );
};

export { TableView as Table };