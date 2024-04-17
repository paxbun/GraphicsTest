import android.content.Context
import android.graphics.Canvas
import android.graphics.PixelFormat
import android.view.Surface
import android.view.SurfaceHolder
import android.view.SurfaceView
import android.widget.LinearLayout
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.viewinterop.AndroidView
import com.sun.jna.Library
import graphics_test.RustNativeViewContext

@Composable
actual fun RustView(modifier: Modifier) {
    AndroidView(
        factory = ::RustNativeView,
        modifier = modifier,
        update = RustNativeView::render
    )
}

private class RustNativeView(context: Context) : LinearLayout(context) {
    private lateinit var nativeViewContext: RustNativeViewContext

    init {
        addView(SurfaceView(context).apply {
            holder.setFormat(PixelFormat.TRANSLUCENT)
            holder.addCallback(object : SurfaceHolder.Callback {
                override fun surfaceCreated(holder: SurfaceHolder) {
                    val nativeHandle =
                        RustNativeViewLib.nativeViewAsNativeHandle(holder.surface).apply {
                            if (this == 0L) {
                                error("Could not create RustNativeViewContext")
                            }
                        }
                    nativeViewContext = RustNativeViewContext(
                        nativeHandle,
                        resources.displayMetrics.density
                    )
                    nativeViewContext.render()
                }

                override fun surfaceChanged(
                    holder: SurfaceHolder,
                    format: Int,
                    width: Int,
                    height: Int
                ) {
                    nativeViewContext.changeSize(width, height, resources.displayMetrics.density)
                    nativeViewContext.render()
                }

                override fun surfaceDestroyed(holder: SurfaceHolder) {
                    nativeViewContext.destroy()
                }

            })
        })
    }

    override fun draw(canvas: Canvas) {
        super.draw(canvas)
        nativeViewContext.render()
    }

    fun render() {
        if (::nativeViewContext.isInitialized) {
            nativeViewContext.render()
        }
    }
}

object RustNativeViewLib : Library {
    init {
        System.loadLibrary("graphics_test")
    }

    @JvmStatic
    external fun nativeViewAsNativeHandle(surface: Surface): Long
}