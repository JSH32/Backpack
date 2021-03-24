import * as React from 'react';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import Button from '@material-ui/core/Button';

import styles from './style.scss';

export const Header = () => {
    return <div>
        <AppBar position="static" color="transparent" elevation={0}>
            <Toolbar>
                <Typography className={styles.title}>
                    KAWAII.SH
                </Typography>
                <Button>Login</Button>
            </Toolbar>
        </AppBar>
    </div>
}