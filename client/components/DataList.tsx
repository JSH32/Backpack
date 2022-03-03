import React from "react"

import { 
    Accordion, 
    AccordionButton, 
    AccordionIcon, 
    AccordionItem, 
    AccordionPanel, 
    AccordionProps, 
    Box, 
    Flex, 
    FlexProps, 
    useBreakpointValue, 
    useColorModeValue 
} from "@chakra-ui/react"

export const DataListContext = React.createContext<any>(null)
export const DataListHeaderContext = React.createContext<any>(null)

export interface DataListCellProps extends FlexProps {
    children?: React.ReactElement | React.ReactElement[],
    colName?: string | any;
    colWidth?: string | number | Record<string, string | number>;
    isVisible?: boolean | boolean[] | Record<string, boolean>;
}

export const DataListCell: React.FC<DataListCellProps> = ({
    children,
    colName,
    colWidth = 1,
    isVisible = true,
    ...rest
}) => {
    const { columns, setColumns } = React.useContext(DataListContext)
    const isInHeader = React.useContext(DataListHeaderContext)
    const restRef = React.useRef<any>()
    restRef.current = rest

    React.useEffect(() => {
        if (isInHeader && colName) {
            setColumns((prevColumns: any) => ({
                ...prevColumns,
                [colName]: { colWidth, isVisible, ...restRef.current }
            }))
        }
    }, [isInHeader, colName, colWidth, isVisible, setColumns])

    const headerProps = !isInHeader ? columns?.[colName] || {} : {}
    const {
        isVisible: _isVisible = true,
        colWidth: _colWidth = true,
        ...cellProps
    } = {
        colWidth,
        isVisible,
        ...headerProps,
        ...rest
    }

    const showCell = useBreakpointValue(
        typeof _isVisible === "object" ? _isVisible : { base: _isVisible }
    )

    const cellWidth = useBreakpointValue(
        typeof _colWidth === "object" ? _colWidth : { base: _colWidth }
    )

    if (!showCell) return null

    const isWidthUnitless = /^[0-9.]+$/.test(cellWidth)

    return (
        <Flex
            direction="column"
            minW={!isWidthUnitless ? cellWidth : 0}
            flexBasis={isWidthUnitless ? `${+cellWidth * 100}%` : cellWidth}
            py="2"
            px="4"
            align="flex-start"
            justifyContent="center"
            {...cellProps}
        >
            {children}
        </Flex>
    )
}

export const DataListAccordion: React.FC = ({ ...rest }) => {
    return <AccordionItem border="none" {...rest} />
}

export const DataListAccordionButton: React.FC = ({ ...rest }) => {
    return (
        <AccordionButton
            role="group"
            p="0"
            textAlign="left"
            _focus={{ outline: "none" }}
            _hover={{}}
            {...rest}
        />
    )
}

export const DataListAccordionIcon: React.FC = ({ ...rest }) => {
    return (
        <AccordionIcon
            borderRadius="full"
            _groupFocus={{ boxShadow: "outline" }}
            {...rest}
        />
    )
}

export const DataListAccordionPanel: React.FC = ({ ...rest }) => {
    return (
        <AccordionPanel
            boxShadow="inner"
            px="4"
            py="3"
            bg={useColorModeValue("gray.50", "blackAlpha.400")}
            {...rest}
        />
    )
}

export interface DataListRowProps extends FlexProps {
    isVisible?: boolean | boolean[] | Record<string, boolean>;
    isDisabled?: boolean;
}

export const DataListRow: React.FC<DataListRowProps> = ({
    isVisible = true,
    isDisabled = false,
    ...rest
}) => {
    const { isHover } = React.useContext(DataListContext)
    const showRow = useBreakpointValue(
        typeof isVisible === "object" ? isVisible : { base: isVisible }
    )
    const disabledProps: any = isDisabled
        ? {
            bg: useColorModeValue("gray.50", "whiteAlpha.50"),
            pointerEvents: "none",
            _hover: {},
            _focus: {},
            "aria-disabled": true,
            opacity: "1 !important",
            css: {
                "> *": {
                    opacity: 0.3
                }
            }
        }
        : {}
    return (
        <Flex
            d={!showRow ? "none" : null}
            position="relative"
            borderBottom="1px solid"
            borderBottomColor={useColorModeValue("gray.100", "gray.900")}
            transition="0.2s"
            _hover={
                isHover ? { bg: useColorModeValue("gray.50", "blackAlpha.200") } : null
            }
            {...disabledProps}
            {...rest}
        />
    )
}

export type DataListHeaderProps = DataListRowProps

export const DataListHeader: React.FC<DataListHeaderProps> = ({ ...rest }) => {
    return (
        <DataListHeaderContext.Provider value={true}>
            <DataListRow
                bg={useColorModeValue("gray.100", "blackAlpha.400")}
                fontSize="sm"
                fontWeight="bold"
                color={useColorModeValue("gray.600", "gray.300")}
                border="none"
                _hover={{}}
                {...rest}
            />
        </DataListHeaderContext.Provider>
    )
}

export type DataListFooterProps = DataListRowProps

export const DataListFooter: React.FC<DataListFooterProps> = ({ ...rest }) => {
    return (
        <Box mt="auto">
            <Flex
                bg={useColorModeValue("white", "blackAlpha.50")}
                fontSize="sm"
                color={useColorModeValue("gray.600", "gray.300")}
                mt="-1px"
                borderTop="1px solid"
                borderTopColor={useColorModeValue("gray.100", "gray.900")}
                p="2"
                align="center"
                {...rest}
            />
        </Box>
    )
}

export interface DataListProps extends AccordionProps {
    isHover?: boolean;
}

export const DataList: React.FC<DataListProps> = ({
    allowMultiple = true,
    allowToggle = false,
    isHover = true,
    ...rest
}) => {
    const [columns, setColumns] = React.useState({})
    return (
        <DataListContext.Provider
            value={{
                setColumns,
                columns,
                isHover
            }}
        >
            <Accordion
                display="flex"
                flexDirection="column"
                bg={useColorModeValue("white", "blackAlpha.300")}
                position="relative"
                boxShadow="md"
                borderRadius="md"
                overflowX="auto"
                overflowY="hidden"
                minH="10rem"
                allowMultiple={allowMultiple && !allowToggle}
                allowToggle={allowToggle}
                {...rest}
            />
        </DataListContext.Provider>
    )
}