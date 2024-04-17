package me.paxbun.graphicstest

import App
import RustView
import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        setContent {
            MaterialTheme {
                var showContent by remember { mutableStateOf(false) }
                Column {
                    Surface(color = Color.Red) {
                        Row {
                            Button({
                                startActivity(
                                    Intent(
                                        this@MainActivity,
                                        NativeViewActivity::class.java
                                    )
                                )
                            }) {
                                Text("NativeView")
                            }
                            Button(onClick = { showContent = !showContent }) {
                                Text("Click me!")
                            }
                        }
                    }
                    AnimatedVisibility(showContent) {
                        RustView(Modifier.fillMaxHeight())
                    }
                }
            }
        }
    }
}

@Preview
@Composable
fun AppAndroidPreview() {
    App()
}